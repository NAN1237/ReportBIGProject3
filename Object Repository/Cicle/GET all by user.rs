<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET all by user</name>
   <tag></tag>
   <elementGuidId>4fc9a73f-74bc-40c9-8fc4-73c6c0da00d6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYzNTAwMGJjMmUyMGQ4OGIyOGI2MWMwMCIsImdvb2dsZUlkIjoiMTAzMzMzNjA3NDgwNjgyNjc3NTI5IiwiZW1haWwiOiJkd2ltYWhlbmRyYW5hbmRhQGdtYWlsLmNvbSIsImZ1bGxOYW1lIjoiTmFuZGEgRHdpIE1haGVuZHJhIiwicGhvdG9VcmwiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BTG01d3UwLUxsV3JYamNzQ0RrRVFpWWV0WUp2c19NTWh6RUplUHI4M3V5TnhBPXM5Ni1jIiwiYmlvIjoiIiwic3RhdHVzIjoiIiwiZGVmYXVsdENvbXBhbnkiOnsiX2lkIjoiNjFlYmEyYzg1MDgwZjQ3YjI1ZGRjOGY4In0sImNyZWF0ZWRBdCI6IjIwMjItMTAtMTlUMTM6NTA6NTIuODU2WiIsInVwZGF0ZWRBdCI6IjIwMjItMTAtMzFUMTQ6Mzk6NDcuMTk1WiIsIl9fdiI6MH0sImlhdCI6MTY2NzU2NjgwMSwiZXhwIjoxNjcwMTU4ODAxfQ.34YW-r3VWMzNetALPO5DfCBV-sUmrXNL5Rz390axdiA</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>9c5d0b76-95eb-4418-9d38-a8457aa8bb3e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>jwt eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7Il9pZCI6IjYzNTAwMGJjMmUyMGQ4OGIyOGI2MWMwMCIsImdvb2dsZUlkIjoiMTAzMzMzNjA3NDgwNjgyNjc3NTI5IiwiZW1haWwiOiJkd2ltYWhlbmRyYW5hbmRhQGdtYWlsLmNvbSIsImZ1bGxOYW1lIjoiTmFuZGEgRHdpIE1haGVuZHJhIiwicGhvdG9VcmwiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BTG01d3UwLUxsV3JYamNzQ0RrRVFpWWV0WUp2c19NTWh6RUplUHI4M3V5TnhBPXM5Ni1jIiwiYmlvIjoiIiwic3RhdHVzIjoiIiwiZGVmYXVsdENvbXBhbnkiOnsiX2lkIjoiNjFlYmEyYzg1MDgwZjQ3YjI1ZGRjOGY4In0sImNyZWF0ZWRBdCI6IjIwMjItMTAtMTlUMTM6NTA6NTIuODU2WiIsInVwZGF0ZWRBdCI6IjIwMjItMTAtMzFUMTQ6Mzk6NDcuMTk1WiIsIl9fdiI6MH0sImlhdCI6MTY2NzU2NjgwMSwiZXhwIjoxNjcwMTU4ODAxfQ.34YW-r3VWMzNetALPO5DfCBV-sUmrXNL5Rz390axdiA</value>
      <webElementGuid>8c050cd7-9fe2-4465-ba10-9f34064ada9d</webElementGuid>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://stagingapi.cicle.app/api/v1/companies</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>b2f14e8e-e388-466f-8f96-e11f636cfeaa</id>
      <name>wrong</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\ASUS\Downloads\JSON\companyscheme Wrong.json</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>d42993a5-9b14-4090-9436-d14b300e1f2e</id>
      <name>Team Validation</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\ASUS\Downloads\JSON\TeamsSchema.json</data>
      <activate>true</activate>
   </validationSteps>
   <validationSteps>
      <id>058fdfb2-af5d-4ec2-9c47-e04fd74eb56c</id>
      <name>right</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>C:\Users\ASUS\Downloads\JSON\companyscheme.json</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>21</defaultValue>
      <description></description>
      <id>0de4fa76-1864-4d5e-aa8a-70ad8f4f4342</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <variables>
      <defaultValue>'ngoc'</defaultValue>
      <description></description>
      <id>086a27a9-05ed-4e07-8465-e17bddfdeac1</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'1234567890'</defaultValue>
      <description></description>
      <id>311c2780-afd0-4352-a23f-fc00c2c42271</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>'https://www.rd.com/wp-content/uploads/2019/06/lily-of-the-valley-760x506.jpg'</defaultValue>
      <description></description>
      <id>bcddbada-1d5e-422c-b810-55f7c48bb931</id>
      <masked>false</masked>
      <name>avatar</name>
   </variables>
   <variables>
      <defaultValue>'FEMALE'</defaultValue>
      <description></description>
      <id>3b3cb3f6-19ff-4f44-ae2c-a1c680bd3044</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.successCode</defaultValue>
      <description></description>
      <id>f1c9042c-8dca-4d38-ba9e-77542f9e8d20</id>
      <masked>false</masked>
      <name>expectedStatusCode</name>
   </variables>
   <variables>
      <defaultValue>7</defaultValue>
      <description></description>
      <id>c4249c68-7dc1-4ffc-ad8e-a4109ce4bb7b</id>
      <masked>false</masked>
      <name>id</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
assert response.getStatusCode() == 200
WS.verifyElementPropertyValue(response, &quot;age&quot;, 25)
WS.verifyElementPropertyValue(response, &quot;username&quot;, &quot;mimi&quot;)
WS.verifyElementPropertyValue(response, &quot;password&quot;, &quot;123456789&quot;)
WS.verifyElementPropertyValue(response, &quot;gender&quot;, &quot;MALE&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
